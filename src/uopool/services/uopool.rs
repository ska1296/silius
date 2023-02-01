use super::sanity_check::UserOperationSanityCheckError;
use crate::{
    contracts::EntryPoint,
    types::{reputation::ReputationEntry, user_operation::UserOperation},
    uopool::{
        mempool_id,
        server::uopool::{
            uo_pool_server::UoPool, AddRequest, AddResponse, AddResult, ClearRequest,
            ClearResponse, ClearResult, GetAllReputationRequest, GetAllReputationResponse,
            GetAllReputationResult, GetAllRequest, GetAllResponse, GetAllResult, RemoveRequest,
            RemoveResponse, SetReputationRequest, SetReputationResponse, SetReputationResult,
        },
        services::sanity_check::SANITY_CHECK_ERROR_CODE,
        MempoolBox, MempoolId, ReputationBox,
    },
};
use async_trait::async_trait;
use ethers::{
    providers::Middleware,
    types::{Address, U256},
};
use jsonrpsee::{
    tracing::info,
    types::{error::ErrorCode, ErrorObject},
};
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};
use tonic::Response;

pub type UoPoolError = ErrorObject<'static>;

pub struct UoPoolService<M: Middleware> {
    pub entry_points: Arc<HashMap<MempoolId, EntryPoint<M>>>,
    pub mempools: Arc<RwLock<HashMap<MempoolId, MempoolBox<Vec<UserOperation>>>>>,
    pub reputations: Arc<RwLock<HashMap<MempoolId, ReputationBox<Vec<ReputationEntry>>>>>,
    pub eth_provider: Arc<M>,
    pub max_verification_gas: U256,
    pub chain_id: U256,
}

impl<M: Middleware> From<UserOperationSanityCheckError<M>> for UoPoolError {
    fn from(_: UserOperationSanityCheckError<M>) -> Self {
        UoPoolError::from(ErrorCode::ServerError(-32602))
    }
}

impl<M: Middleware + 'static> UoPoolService<M> {
    pub fn new(
        entry_points: Arc<HashMap<MempoolId, EntryPoint<M>>>,
        mempools: Arc<RwLock<HashMap<MempoolId, MempoolBox<Vec<UserOperation>>>>>,
        reputations: Arc<RwLock<HashMap<MempoolId, ReputationBox<Vec<ReputationEntry>>>>>,
        eth_provider: Arc<M>,
        max_verification_gas: U256,
        chain_id: U256,
    ) -> Self {
        Self {
            entry_points,
            mempools,
            reputations,
            eth_provider,
            max_verification_gas,
            chain_id,
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> UoPool for UoPoolService<M> {
    async fn add(
        &self,
        request: tonic::Request<AddRequest>,
    ) -> Result<Response<AddResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut res = AddResponse::default();

        if let AddRequest {
            uo: Some(user_operation),
            ep: Some(entry_point),
        } = req
        {
            let user_operation: UserOperation = user_operation
                .try_into()
                .map_err(|_| tonic::Status::invalid_argument("invalid user operation"))?;
            let entry_point: Address = entry_point
                .try_into()
                .map_err(|_| tonic::Status::invalid_argument("invalid entry point"))?;

            info!("{:?}", user_operation);
            info!("{:?}", entry_point);

            //  sanity check
            match self.validate_user_operation(&user_operation).await {
                Ok(_) => {
                    // simulation

                    // TODO: make something with reputation

                    // add to mempool

                    res.set_result(AddResult::Added);
                    res.data =
                        serde_json::to_string(&user_operation.hash(&entry_point, &self.chain_id))
                            .map_err(|_| tonic::Status::internal("error adding user operation"))?;
                }
                Err(error) => match error {
                    UserOperationSanityCheckError::SanityCheck(user_operation_error) => {
                        res.set_result(AddResult::NotAdded);
                        res.data = serde_json::to_string(&UoPoolError::owned::<String>(
                            SANITY_CHECK_ERROR_CODE,
                            user_operation_error.to_string(),
                            None,
                        ))
                        .map_err(|_| tonic::Status::internal("error adding user operation"))?;
                    }
                    _ => {
                        return Err(tonic::Status::internal("error adding user operation"));
                    }
                },
            }

            return Ok(tonic::Response::new(res));
        }

        Err(tonic::Status::invalid_argument("missing user operation"))
    }

    async fn remove(
        &self,
        _request: tonic::Request<RemoveRequest>,
    ) -> Result<Response<RemoveResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("todo"))
    }

    #[cfg(debug_assertions)]
    async fn clear(
        &self,
        _request: tonic::Request<ClearRequest>,
    ) -> Result<Response<ClearResponse>, tonic::Status> {
        for mempool in self.mempools.write().values_mut() {
            mempool.clear();
        }

        for reputation in self.reputations.write().values_mut() {
            reputation.clear();
        }

        Ok(tonic::Response::new(ClearResponse {
            result: ClearResult::Cleared as i32,
        }))
    }

    #[cfg(debug_assertions)]
    async fn get_all(
        &self,
        request: tonic::Request<GetAllRequest>,
    ) -> Result<Response<GetAllResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut res = GetAllResponse::default();

        if let Some(entry_point) = req.ep {
            let entry_point: Address = entry_point
                .try_into()
                .map_err(|_| tonic::Status::invalid_argument("invalid entry point"))?;

            if let Some(mempool) = self
                .mempools
                .read()
                .get(&mempool_id(entry_point, self.chain_id))
            {
                res.result = GetAllResult::GotAll as i32;
                res.uos = mempool
                    .get_all()
                    .iter()
                    .map(|uo| uo.clone().into())
                    .collect();
            } else {
                res.result = GetAllResult::NotGotAll as i32;
            }

            return Ok(tonic::Response::new(res));
        }

        Err(tonic::Status::invalid_argument("missing entry point"))
    }

    #[cfg(debug_assertions)]
    async fn set_reputation(
        &self,
        request: tonic::Request<SetReputationRequest>,
    ) -> Result<Response<SetReputationResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut res = SetReputationResponse::default();

        if let Some(entry_point) = req.ep {
            let entry_point: Address = entry_point
                .try_into()
                .map_err(|_| tonic::Status::invalid_argument("invalid entry point"))?;

            if let Some(reputation) = self
                .reputations
                .write()
                .get_mut(&mempool_id(entry_point, self.chain_id))
            {
                reputation.set(req.res.iter().map(|re| re.clone().into()).collect());
                res.result = SetReputationResult::SetReputation as i32;
            } else {
                res.result = SetReputationResult::NotSetReputation as i32;
            }

            return Ok(tonic::Response::new(res));
        }

        Err(tonic::Status::invalid_argument("missing entry point"))
    }

    #[cfg(debug_assertions)]
    async fn get_all_reputation(
        &self,
        request: tonic::Request<GetAllReputationRequest>,
    ) -> Result<Response<GetAllReputationResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut res = GetAllReputationResponse::default();

        if let Some(entry_point) = req.ep {
            let entry_point: Address = entry_point
                .try_into()
                .map_err(|_| tonic::Status::invalid_argument("invalid entry point"))?;

            if let Some(reputation) = self
                .reputations
                .read()
                .get(&mempool_id(entry_point, self.chain_id))
            {
                res.result = GetAllReputationResult::GotAllReputation as i32;
                res.res = reputation.get_all().iter().map(|re| (*re).into()).collect();
            } else {
                res.result = GetAllReputationResult::NotGotAllReputation as i32;
            }

            return Ok(tonic::Response::new(res));
        };

        Err(tonic::Status::invalid_argument("missing entry point"))
    }
}
