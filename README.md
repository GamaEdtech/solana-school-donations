# Solana School Donation

A fully on-chain donation system built with **Solana** and **Anchor**, designed to support schools and students in a transparent and decentralized way.  
Donors can contribute using **$GET token** or **SOL**, and all requests and contributions are recorded on-chain for full accountability.

---

## Features

-  **On-chain Requests** – Schools or representatives can create requests for equipment or resources.
-  **Transparent Donations** – Donors fund requests directly on-chain with $GET token or SOL.
-  **Track Progress** – Each request tracks partial or full funding status.
-  **Trustless & Secure** – Smart contract logic ensures integrity of funds and prevents misuse.
-  **Audit & History** – Every donation is permanently recorded on Solana blockchain.
-  **Low-Cost** – Powered by Solana’s fast and low-fee transactions.
-  **Future-Ready** – Built for DAO governance and community-driven expansion.

---

## Architecture

The program uses **Anchor framework** to manage accounts and instructions:

- **School Account** – Represents a school (linked to public key).
- **Request Account** – Equipment or funding request linked to a school.
- **Donation Account** – Tracks each donor’s contribution toward a request.
- **State Transitions** – Request can move through statuses: `Pending → Promoted → Funded → Fulfilled`.

---

## Data Model

- **School**
  - `id`
  - `name`
  - `authority (PublicKey)`
  - `metadata_uri`

- **Request**
  - `id`
  - `school_id`
  - `description`
  - `quantity`
  - `estimated_cost`
  - `status` (Pending, Promoted, Funded, Fulfilled)
  - `created_at`

- **Donation**
  - `id`
  - `request_id`
  - `donor (PublicKey)`
  - `amount`
  - `tx_signature`
  - `created_at`

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli)
- [Anchor](https://www.anchor-lang.com/docs/installation)

### Installation
```bash
# Clone repo
git clone https://github.com/YOUR-USERNAME/solana-school-donation.git
cd solana-school-donation

# Install dependencies
anchor build
