use alloy::{
    eips::eip2718::Encodable2718,
    network::Network,
    primitives::B256,
    providers::{
        fillers::{FillProvider, TxFiller},
        Provider,
    },
    transports::{http::Http, TransportResult},
};
use alloy_rpc_types::mev::{
    CancelBundleRequest, EthCallBundle, EthCallBundleResponse, EthSendBundle,
    PrivateTransactionRequest, SendBundleResponse,
};
use async_trait::async_trait;

use crate::http::{BroadcastableCall, Endpoints, EndpointsBuilder};

/// Extension trait for sending and simulate eth bundles.
#[async_trait]
pub trait EthProviderExt<C, N>
where
    N: Network,
{
    /// Submits a bundle to one or more builder(s). It takes in a bundle and
    /// provides a bundle hash as a return value.
    async fn send_eth_bundle(
        &self,
        bundle: EthSendBundle,
        endpoints: &Endpoints<C>,
    ) -> Vec<TransportResult<SendBundleResponse>>;

    /// Submits a single transaction to one or more builder(s). It takes in a
    /// bundle and provides a bundle hash as a return value.
    async fn send_eth_private_transaction(
        &self,
        request: PrivateTransactionRequest,
    ) -> TransportResult<B256>;

    /// simulates a bundle against a specific block number.
    async fn call_eth_bundle(
        &self,
        bundle: EthCallBundle,
    ) -> TransportResult<EthCallBundleResponse>;

    /// Cancels a previously submitted bundle.
    async fn cancel_eth_bundle(&self, request: CancelBundleRequest) -> TransportResult<()>;

    /// Returns a [`EndpointsBuilder`] that can be used to build a new
    /// [`Endpoints`].
    fn endpoints_builder(&self) -> EndpointsBuilder<C>;
}

#[async_trait]
impl<F, P, N> EthProviderExt<reqwest::Client, N> for FillProvider<F, P, Http<reqwest::Client>, N>
where
    F: TxFiller<N>,
    P: Provider<Http<reqwest::Client>, N>,
    N: Network,
    <N as Network>::TxEnvelope: Encodable2718 + Clone,
{
    async fn send_eth_bundle(
        &self,
        bundle: EthSendBundle,
        endpoints: &Endpoints<reqwest::Client>,
    ) -> Vec<TransportResult<SendBundleResponse>> {
        BroadcastableCall::new(
            endpoints,
            self.client().make_request("eth_sendBundle", (bundle,)),
        )
        .await
    }

    async fn send_eth_private_transaction(
        &self,
        request: PrivateTransactionRequest,
    ) -> TransportResult<B256> {
        self.client()
            .request("eth_sendPrivateTransaction", (request,))
            .await
    }

    async fn call_eth_bundle(
        &self,
        bundle: EthCallBundle,
    ) -> TransportResult<EthCallBundleResponse> {
        self.client().request("eth_callBundle", (bundle,)).await
    }

    async fn cancel_eth_bundle(&self, request: CancelBundleRequest) -> TransportResult<()> {
        self.client().request("eth_cancelBundle", (request,)).await
    }

    fn endpoints_builder(&self) -> EndpointsBuilder<reqwest::Client> {
        EndpointsBuilder::new(self.client().transport().clone())
    }
}
