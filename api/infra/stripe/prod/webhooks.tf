terraform {
  required_providers {
    stripe = {
      source = "lukasaron/stripe"
      version = "1.9.3"
    }
  }
}

provider "stripe" {
  api_key = "<API_KEY>"
}

resource "stripe_webhook_endpoint" "confirm_quotation_payment_webhook_prod" {
  url            = "https://api.rusticad.com/api/v1/quotations/webhooks/confirm_payment"
  description    = "Webhook used by Stripe to confirm when a quotation payment is successful."
  enabled_events = [
    "checkout.session.completed"
  ]
}
