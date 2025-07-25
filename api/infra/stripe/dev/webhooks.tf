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
  url         = "https://httpbin.org/status/200"
  description = "Webhook used by Stripe to confirm when a quotation payment is successful."
  enabled_events = [
    "checkout.session.completed"
  ]
}
