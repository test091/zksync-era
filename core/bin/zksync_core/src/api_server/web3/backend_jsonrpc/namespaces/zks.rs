// Built-in uses
use std::collections::HashMap;

// External uses
use bigdecimal::BigDecimal;
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

// Workspace uses
use zksync_types::{
    api::{BridgeAddresses, L2ToL1LogProof, TransactionDetails},
    explorer_api::{BlockDetails, L1BatchDetails},
    fee::Fee,
    transaction_request::CallRequest,
    Address, Bytes, L1BatchNumber, MiniblockNumber, H256, U256, U64,
};
use zksync_web3_decl::error::Web3Error;
use zksync_web3_decl::types::Token;

// Local uses
use crate::web3::namespaces::ZksNamespace;
use crate::{l1_gas_price::L1GasPriceProvider, web3::backend_jsonrpc::error::into_jsrpc_error};

#[rpc]
pub trait ZksNamespaceT {
    #[rpc(name = "zks_estimateFee")]
    fn estimate_fee(&self, req: CallRequest) -> BoxFuture<Result<Fee>>;

    #[rpc(name = "zks_estimateGasL1ToL2")]
    fn estimate_gas_l1_to_l2(&self, req: CallRequest) -> BoxFuture<Result<U256>>;

    #[rpc(name = "zks_getMainContract")]
    fn get_main_contract(&self) -> BoxFuture<Result<Address>>;

    #[rpc(name = "zks_getTestnetPaymaster")]
    fn get_testnet_paymaster(&self) -> BoxFuture<Result<Option<Address>>>;

    #[rpc(name = "zks_getBridgeContracts")]
    fn get_bridge_contracts(&self) -> BoxFuture<Result<BridgeAddresses>>;

    #[rpc(name = "zks_L1ChainId")]
    fn l1_chain_id(&self) -> BoxFuture<Result<U64>>;

    #[rpc(name = "zks_getConfirmedTokens")]
    fn get_confirmed_tokens(&self, from: u32, limit: u8) -> BoxFuture<Result<Vec<Token>>>;

    #[rpc(name = "zks_getTokenPrice")]
    fn get_token_price(&self, token_address: Address) -> BoxFuture<Result<BigDecimal>>;

    #[rpc(name = "zks_getAllAccountBalances")]
    fn get_all_account_balances(
        &self,
        address: Address,
    ) -> BoxFuture<Result<HashMap<Address, U256>>>;

    #[rpc(name = "zks_getL2ToL1MsgProof")]
    fn get_l2_to_l1_msg_proof(
        &self,
        block: MiniblockNumber,
        sender: Address,
        msg: H256,
        l2_log_position: Option<usize>,
    ) -> BoxFuture<Result<Option<L2ToL1LogProof>>>;

    #[rpc(name = "zks_getL2ToL1LogProof")]
    fn get_l2_to_l1_log_proof(
        &self,
        tx_hash: H256,
        index: Option<usize>,
    ) -> BoxFuture<Result<Option<L2ToL1LogProof>>>;

    #[rpc(name = "zks_L1BatchNumber")]
    fn get_l1_batch_number(&self) -> BoxFuture<Result<U64>>;

    #[rpc(name = "zks_getBlockDetails")]
    fn get_block_details(
        &self,
        block_number: MiniblockNumber,
    ) -> BoxFuture<Result<Option<BlockDetails>>>;

    #[rpc(name = "zks_getL1BatchBlockRange")]
    fn get_miniblock_range(&self, batch: L1BatchNumber) -> BoxFuture<Result<Option<(U64, U64)>>>;

    #[rpc(name = "zks_setKnownBytecode")]
    fn set_known_bytecode(&self, bytecode: Bytes) -> BoxFuture<Result<bool>>;

    #[rpc(name = "zks_getTransactionDetails")]
    fn get_transaction_details(&self, hash: H256) -> BoxFuture<Result<Option<TransactionDetails>>>;

    #[rpc(name = "zks_getRawBlockTransactions")]
    fn get_raw_block_transactions(
        &self,
        block_number: MiniblockNumber,
    ) -> BoxFuture<Result<Vec<zksync_types::Transaction>>>;

    #[rpc(name = "zks_getL1BatchDetails")]
    fn get_l1_batch_details(
        &self,
        batch: L1BatchNumber,
    ) -> BoxFuture<Result<Option<L1BatchDetails>>>;

    #[rpc(name = "zks_getBytecodeByHash")]
    fn get_bytecode_by_hash(&self, hash: H256) -> BoxFuture<Result<Option<Vec<u8>>>>;

    #[rpc(name = "zks_getL1GasPrice")]
    fn get_l1_gas_price(&self) -> BoxFuture<Result<U64>>;
}

impl<G: L1GasPriceProvider + Send + Sync + 'static> ZksNamespaceT for ZksNamespace<G> {
    fn estimate_fee(&self, req: CallRequest) -> BoxFuture<Result<Fee>> {
        let self_ = self.clone();
        Box::pin(async move { self_.estimate_fee_impl(req).await.map_err(into_jsrpc_error) })
    }

    fn estimate_gas_l1_to_l2(&self, req: CallRequest) -> BoxFuture<Result<U256>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .estimate_l1_to_l2_gas_impl(req)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_main_contract(&self) -> BoxFuture<Result<Address>> {
        let self_ = self.clone();
        Box::pin(async move { Ok(self_.get_main_contract_impl()) })
    }

    fn get_miniblock_range(&self, batch: L1BatchNumber) -> BoxFuture<Result<Option<(U64, U64)>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_miniblock_range_impl(batch)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_testnet_paymaster(&self) -> BoxFuture<Result<Option<Address>>> {
        let self_ = self.clone();
        Box::pin(async move { Ok(self_.get_testnet_paymaster_impl()) })
    }

    fn get_bridge_contracts(&self) -> BoxFuture<Result<BridgeAddresses>> {
        let self_ = self.clone();
        Box::pin(async move { Ok(self_.get_bridge_contracts_impl()) })
    }

    fn l1_chain_id(&self) -> BoxFuture<Result<U64>> {
        let self_ = self.clone();
        Box::pin(async move { Ok(self_.l1_chain_id_impl()) })
    }

    fn get_confirmed_tokens(&self, from: u32, limit: u8) -> BoxFuture<Result<Vec<Token>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_confirmed_tokens_impl(from, limit)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_token_price(&self, token_address: Address) -> BoxFuture<Result<BigDecimal>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_token_price_impl(token_address)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_all_account_balances(
        &self,
        address: Address,
    ) -> BoxFuture<Result<HashMap<Address, U256>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_all_account_balances_impl(address)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_l2_to_l1_msg_proof(
        &self,
        block: MiniblockNumber,
        sender: Address,
        msg: H256,
        l2_log_position: Option<usize>,
    ) -> BoxFuture<Result<Option<L2ToL1LogProof>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_l2_to_l1_msg_proof_impl(block, sender, msg, l2_log_position)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_l2_to_l1_log_proof(
        &self,
        tx_hash: H256,
        index: Option<usize>,
    ) -> BoxFuture<Result<Option<L2ToL1LogProof>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_l2_to_l1_log_proof_impl(tx_hash, index)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_l1_batch_number(&self) -> BoxFuture<Result<U64>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_l1_batch_number_impl()
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_block_details(
        &self,
        block_number: MiniblockNumber,
    ) -> BoxFuture<Result<Option<BlockDetails>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_block_details_impl(block_number)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_transaction_details(&self, hash: H256) -> BoxFuture<Result<Option<TransactionDetails>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_transaction_details_impl(hash)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn set_known_bytecode(&self, _bytecode: Bytes) -> BoxFuture<Result<bool>> {
        #[cfg(feature = "openzeppelin_tests")]
        let self_ = self.clone();
        Box::pin(async move {
            #[cfg(feature = "openzeppelin_tests")]
            return Ok(self_.set_known_bytecode_impl(_bytecode));

            #[cfg(not(feature = "openzeppelin_tests"))]
            Err(into_jsrpc_error(Web3Error::NotImplemented))
        })
    }

    fn get_raw_block_transactions(
        &self,
        block_number: MiniblockNumber,
    ) -> BoxFuture<Result<Vec<zksync_types::Transaction>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_raw_block_transactions_impl(block_number)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_l1_batch_details(
        &self,
        batch: L1BatchNumber,
    ) -> BoxFuture<Result<Option<L1BatchDetails>>> {
        let self_ = self.clone();
        Box::pin(async move {
            self_
                .get_l1_batch_details_impl(batch)
                .await
                .map_err(into_jsrpc_error)
        })
    }

    fn get_bytecode_by_hash(&self, hash: H256) -> BoxFuture<Result<Option<Vec<u8>>>> {
        let self_ = self.clone();
        Box::pin(async move { Ok(self_.get_bytecode_by_hash_impl(hash).await) })
    }

    fn get_l1_gas_price(&self) -> BoxFuture<Result<U64>> {
        let self_ = self.clone();
        Box::pin(async move { Ok(self_.get_l1_gas_price_impl()) })
    }
}
