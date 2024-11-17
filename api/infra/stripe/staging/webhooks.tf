terraform {
  required_providers {
    stripe = {
      source  = "lukasaron/stripe"
      version = "1.9.3"
    }
  }
}

provider "stripe" {
  api_key = "<API_KEY>"
}

resource "stripe_webhook_endpoint" "complete_checkout_session_webhook_staging" {
  url         = "https://api.torvek-staging.com/api/v1/payments/webhooks/complete_checkout_session"
  description = "Webhook used by Stripe to confirm when a quotation payment is successful."
  enabled_events = [
    "checkout.session.completed"
  ]
}
