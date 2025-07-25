/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more.
 *
 * The version of the OpenAPI document: v1.3.8
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UiNodeAttributes {
    #[serde(rename = "input")]
    Input(Box<models::UiNodeInputAttributes>),
    #[serde(rename = "text")]
    Text(Box<models::UiNodeTextAttributes>),
    #[serde(rename = "img")]
    Img(Box<models::UiNodeImageAttributes>),
    #[serde(rename = "a")]
    A(Box<models::UiNodeAnchorAttributes>),
    #[serde(rename = "script")]
    Script(Box<models::UiNodeScriptAttributes>),
}

impl Default for UiNodeAttributes {
    fn default() -> Self {
        Self::Input(Default::default())
    }
}

/// The autocomplete attribute for the input. email InputAttributeAutocompleteEmail tel InputAttributeAutocompleteTel url InputAttributeAutocompleteUrl current-password InputAttributeAutocompleteCurrentPassword new-password InputAttributeAutocompleteNewPassword one-time-code InputAttributeAutocompleteOneTimeCode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutocompleteEnum {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "current-password")]
    CurrentPassword,
    #[serde(rename = "new-password")]
    NewPassword,
    #[serde(rename = "one-time-code")]
    OneTimeCode,
}

impl Default for AutocompleteEnum {
    fn default() -> AutocompleteEnum {
        Self::Email
    }
}
/// OnClickTrigger may contain a WebAuthn trigger which should be executed on click.  The trigger maps to a JavaScript function provided by Ory, which triggers actions such as PassKey registration or login. oryWebAuthnRegistration WebAuthnTriggersWebAuthnRegistration oryWebAuthnLogin WebAuthnTriggersWebAuthnLogin oryPasskeyLogin WebAuthnTriggersPasskeyLogin oryPasskeyLoginAutocompleteInit WebAuthnTriggersPasskeyLoginAutocompleteInit oryPasskeyRegistration WebAuthnTriggersPasskeyRegistration oryPasskeySettingsRegistration WebAuthnTriggersPasskeySettingsRegistration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnclickTriggerEnum {
    #[serde(rename = "oryWebAuthnRegistration")]
    OryWebAuthnRegistration,
    #[serde(rename = "oryWebAuthnLogin")]
    OryWebAuthnLogin,
    #[serde(rename = "oryPasskeyLogin")]
    OryPasskeyLogin,
    #[serde(rename = "oryPasskeyLoginAutocompleteInit")]
    OryPasskeyLoginAutocompleteInit,
    #[serde(rename = "oryPasskeyRegistration")]
    OryPasskeyRegistration,
    #[serde(rename = "oryPasskeySettingsRegistration")]
    OryPasskeySettingsRegistration,
}

impl Default for OnclickTriggerEnum {
    fn default() -> OnclickTriggerEnum {
        Self::OryWebAuthnRegistration
    }
}
/// OnLoadTrigger may contain a WebAuthn trigger which should be executed on load.  The trigger maps to a JavaScript function provided by Ory, which triggers actions such as PassKey registration or login. oryWebAuthnRegistration WebAuthnTriggersWebAuthnRegistration oryWebAuthnLogin WebAuthnTriggersWebAuthnLogin oryPasskeyLogin WebAuthnTriggersPasskeyLogin oryPasskeyLoginAutocompleteInit WebAuthnTriggersPasskeyLoginAutocompleteInit oryPasskeyRegistration WebAuthnTriggersPasskeyRegistration oryPasskeySettingsRegistration WebAuthnTriggersPasskeySettingsRegistration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnloadTriggerEnum {
    #[serde(rename = "oryWebAuthnRegistration")]
    OryWebAuthnRegistration,
    #[serde(rename = "oryWebAuthnLogin")]
    OryWebAuthnLogin,
    #[serde(rename = "oryPasskeyLogin")]
    OryPasskeyLogin,
    #[serde(rename = "oryPasskeyLoginAutocompleteInit")]
    OryPasskeyLoginAutocompleteInit,
    #[serde(rename = "oryPasskeyRegistration")]
    OryPasskeyRegistration,
    #[serde(rename = "oryPasskeySettingsRegistration")]
    OryPasskeySettingsRegistration,
}

impl Default for OnloadTriggerEnum {
    fn default() -> OnloadTriggerEnum {
        Self::OryWebAuthnRegistration
    }
}
