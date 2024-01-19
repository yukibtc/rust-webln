// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]
#![allow(clippy::new_without_default)]

use std::ops::Deref;

use wasm_bindgen::prelude::*;
use webln::WebLN;

pub mod error;
pub mod get_info;
pub mod keysend;
pub mod send_payment;

use self::error::{into_err, Result};
use self::get_info::JsGetInfoResponse;
use self::keysend::JsKeysendArgs;
use self::send_payment::JsSendPaymentResponse;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// WebLN instance
#[wasm_bindgen(js_name = WebLN)]
pub struct JsWebLN {
    inner: WebLN,
}

#[wasm_bindgen(js_class = WebLN)]
impl JsWebLN {
    /// Compose new WebLN instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsWebLN> {
        Ok(Self {
            inner: WebLN::new().map_err(into_err)?,
        })
    }

    /// Check if `webln` is enabled without explicitly enabling it through `webln.enable()`
    /// (which may cause a confirmation popup in some providers)
    #[wasm_bindgen(js_name = isEnabled)]
    pub async fn is_enabled(&self) -> Result<bool> {
        self.inner.is_enabled().await.map_err(into_err)
    }

    /// To begin interacting with WebLN APIs you'll first need to enable the provider.
    /// Calling `webln.enable()` will prompt the user for permission to use the WebLN capabilities of the browser.
    /// After that you are free to call any of the other API methods.
    pub async fn enable(&self) -> Result<()> {
        self.inner.enable().await.map_err(into_err)
    }

    /// Get information about the connected node and what WebLN methods it supports.
    #[wasm_bindgen(js_name = getInfo)]
    pub async fn get_info(&self) -> Result<JsGetInfoResponse> {
        Ok(self.inner.get_info().await.map_err(into_err)?.into())
    }

    /// Request the user to send a keysend payment.
    /// This is a spontaneous payment that does not require an invoice and only needs a destination public key and and amount.
    pub async fn keysend(&self, args: &JsKeysendArgs) -> Result<JsSendPaymentResponse> {
        Ok(self
            .inner
            .keysend(args.deref())
            .await
            .map_err(into_err)?
            .into())
    }

    // TODO: add `make_invoice`

    // Request that the user sends a payment for an invoice.
    #[wasm_bindgen(js_name = sendPayment)]
    pub async fn send_payment(&self, invoice: String) -> Result<JsSendPaymentResponse> {
        Ok(self
            .inner
            .send_payment(invoice)
            .await
            .map_err(into_err)?
            .into())
    }
}
