use std::collections::HashMap;

pub enum Message {
    #[allow(dead_code)]
    SwitchTrusted(bool),
    SwitchBlocked(bool),
    SwitchActive(bool),
    SwitchActiveSpinner(bool),
    SwitchName(String, Option<String>),
    SwitchRssi(String, i32),
    SwitchPage(Option<String>, Option<String>),
    RemoveDevice(String),
    AddRow(bluer::Device),
    SwitchAdapterPowered(bool),
    SwitchAdapterTimeout(u32),
    SwitchAdapterDiscoverable(bool),
    SwitchAdapterName(String, String),
    PopulateAdapterExpander(HashMap<String, String>),
    PopupError(String, adw::ToastPriority),
    UpdateListBoxImage(),
    RequestPinCode(bluer::agent::RequestPinCode),
    DisplayPinCode(bluer::agent::DisplayPinCode),
    RequestPassKey(bluer::agent::RequestPasskey),
    DisplayPassKey(bluer::agent::DisplayPasskey),
    RequestConfirmation(bluer::agent::RequestConfirmation),
    RequestAuthorization(bluer::agent::RequestAuthorization),
    AuthorizeService(bluer::agent::AuthorizeService),
    GoToBluetoothSettings(bool),
    RequestYesNo(String, String, String, adw::ResponseAppearance),
    InvalidateSort(),
    RefreshDevicesList(),
    StartTransfer(String, String, f32, f32, f32),
    UpdateTransfer(String, f32, String),
    RemoveTransfer(String),
} 
