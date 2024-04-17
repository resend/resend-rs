use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::{Config, Result};
use crate::types::{AudienceId, Contact, ContactChanges, ContactData, ContactId};

/// `Resend` APIs for `/audiences/:id/contacts` endpoints.
#[derive(Clone)]
pub struct ContactsService(pub(crate) Arc<Config>);

impl ContactsService {
    /// Creates a contact inside an audience.
    ///
    /// Returns a contact id.
    ///
    /// <https://resend.com/docs/api-reference/contacts/create-contact>
    #[maybe_async::maybe_async]
    pub async fn create(&self, audience: &AudienceId, contact: ContactData) -> Result<ContactId> {
        let path = format!("/audiences/{audience}/contacts");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&contact)).await?;
        let content = response.json::<types::CreateContactResponse>().await?;

        Ok(content.id)
    }

    /// Retrieves a single contact from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact>
    #[maybe_async::maybe_async]
    pub async fn get(&self, contact: &ContactId, audience: &AudienceId) -> Result<Contact> {
        let path = format!("/audiences/{audience}/contacts/{contact}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Contact>().await?;

        Ok(content)
    }

    /// Updates an existing contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/update-contact>
    #[maybe_async::maybe_async]
    pub async fn update(
        &self,
        contact: &ContactId,
        audience: &AudienceId,
        update: ContactChanges,
    ) -> Result<()> {
        let path = format!("/audiences/{audience}/contacts/{contact}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let _content = response.json::<types::UpdateContactResponse>().await?;

        Ok(())
    }

    /// Removes an existing contact from an audience by their email or ID.
    ///
    /// <https://resend.com/docs/api-reference/contacts/delete-contact>
    #[maybe_async::maybe_async]
    pub async fn delete<T>(&self, audience: &AudienceId, email_or_id: &T) -> Result<()>
        where
            T: AsRef<str> + Sync,
    {
        let email_or_id = email_or_id.as_ref();
        let path = format!("/audiences/{audience}/contacts/{email_or_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = self.0.send(request).await?;

        Ok(())
    }

    /// Retrieves all contacts from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contacts>
    #[maybe_async::maybe_async]
    pub async fn list(&self, audience: &AudienceId) -> Result<Vec<Contact>> {
        let path = format!("/audiences/{audience}/contacts");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<types::ListContactResponse>().await?;

        Ok(content.data)
    }
}

impl fmt::Debug for ContactsService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use std::fmt;

    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

    /// Unique [`Contact`] identifier.
    #[derive(Debug, Clone, Deserialize)]
    pub struct ContactId(EcoString);

    impl ContactId {
        /// Creates a new [`ContactId`].
        pub fn new(id: &str) -> Self {
            Self(EcoString::from(id))
        }
    }

    impl AsRef<str> for ContactId {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    impl fmt::Display for ContactId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.as_ref(), f)
        }
    }

    /// Details of a new [`Contact`].
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct ContactData {
        /// Email address of the contact.
        pub email: String,

        /// First name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        /// Last name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        /// Indicates if the contact is unsubscribed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<bool>,
    }

    impl ContactData {
        /// Creates a new [`ContactData`].
        pub fn new(email: &str) -> Self {
            Self {
                email: email.to_owned(),
                first_name: None,
                last_name: None,
                unsubscribed: None,
            }
        }

        /// Adds the first name to the contact.
        #[inline]
        pub fn with_first_name(mut self, name: &str) -> Self {
            self.first_name = Some(name.to_owned());
            self
        }

        /// Adds the last name to the contact.
        #[inline]
        pub fn with_last_name(mut self, name: &str) -> Self {
            self.last_name = Some(name.to_owned());
            self
        }

        /// Toggles the unsubscribe status to `unsubscribe`.
        #[inline]
        pub fn with_unsubscribed(mut self, unsubscribed: bool) -> Self {
            self.unsubscribed = Some(unsubscribed);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateContactResponse {
        /// Unique identifier for the created contact.
        pub id: ContactId,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListContactResponse {
        /// Array containing contact information.
        pub data: Vec<Contact>,
    }

    /// Details of an existing contact.
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Contact {
        /// Unique identifier for the contact.
        pub id: ContactId,
        /// Email address of the contact.
        pub email: String,
        /// First name of the contact.
        pub first_name: String,
        /// Last name of the contact.
        pub last_name: String,
        /// Indicates if the contact is unsubscribed.
        pub unsubscribed: bool,
        /// Timestamp indicating when the contact was created in ISO8601 format.
        pub created_at: String,
    }

    /// List of changes to apply to a [`Contact`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct ContactChanges {
        /// Email address of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        /// First name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        /// Last name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        /// Indicates the subscription status of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<bool>,
    }

    impl ContactChanges {
        /// Creates a new [`ContactChanges`].
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        /// Updates the emails of the contact.
        #[inline]
        pub fn with_email(mut self, email: &str) -> Self {
            self.email = Some(email.to_owned());
            self
        }

        /// Updates the first name of the contact.
        #[inline]
        pub fn with_first_name(mut self, name: &str) -> Self {
            self.first_name = Some(name.to_owned());
            self
        }

        /// Updates the last name of the contact.
        #[inline]
        pub fn with_last_name(mut self, name: &str) -> Self {
            self.last_name = Some(name.to_owned());
            self
        }

        /// Updates the unsubscribe status of the contact.
        #[inline]
        pub fn with_unsubscribed(mut self, unsubscribed: bool) -> Self {
            self.unsubscribed = Some(unsubscribed);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateContactResponse {
        /// Unique identifier for the updated contact.
        pub id: ContactId,
    }
}

#[cfg(test)]
mod test {
    use crate::{Client, Result};
    use crate::types::{ContactChanges, ContactData};

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> Result<()> {
        let resend = Client::default();
        let audience = "test_contacts";

        // Create audience.
        let audience_id = resend.audiences.create(audience).await?;

        // Create.
        let contact = ContactData::new("antonios.barotsis@pm.me")
            .with_first_name("Antonios")
            .with_last_name("Barotsis")
            .with_unsubscribed(false);
        let id = resend.contacts.create(&audience_id, contact).await?;

        // Update.
        let changes = ContactChanges::new().with_unsubscribed(true);
        resend.contacts.update(&id, &audience_id, changes).await?;

        // Retrieve.
        let contact = resend.contacts.get(&id, &audience_id).await?;
        assert!(contact.unsubscribed);

        // List.
        let contacts = resend.contacts.list(&audience_id).await?;
        assert_eq!(contacts.len(), 1);

        // Delete.
        resend.contacts.delete(&audience_id, &id).await?;

        // Delete audience.
        let _ = resend.audiences.delete(&audience_id).await?;

        Ok(())
    }
}
