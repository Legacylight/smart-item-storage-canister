# Smart Item Storage Canister

This repository contains the source code for a smart item storage canister on the Internet Computer (IC). The canister allows users to manage and query a collection of smart storage items, providing functionality such as adding, updating, deleting, and querying items based on various criteria.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
  - [Query Functions](#query-functions)
  - [Update Functions](#update-functions)
- [Testing](#testing)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [License](#license)

## Overview

The smart item storage canister is designed to be a decentralized storage solution for managing information about various items. Each item is represented by the `SmartStorageItem` struct, which includes fields such as `id`, `name`, `description`, `location`, `created_at`, `updated_at`, and `is_available`. The canister uses a BTreeMap for efficient storage and retrieval of items.

Key Features:

- **Querying**: Retrieve information about specific items, all items, available items, or perform a search based on a query string.
- **Updating Availability**: Mark items as available or unavailable.
- **Statistics**: Obtain statistics about the stored items, including total items and average availability rate.
- **History**: View the history of changes for a specific item.
- **Batch Queries**: Perform multiple queries in a single batch.

## Prerequisites

Before you begin, ensure that you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Internet Computer SDK](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html)

## Installation

Clone the repository to your local machine:

```bash
git clone https://github.com/Legacylight/smart-item-storage-canister.git
cd smart-item-storage-canister
```

## Usage

To use the smart item storage canister, you can explore the provided query and update functions:

### Query Functions

- **get_smart_storage_item(id: u64):** Retrieve information about a specific item.
- **get_all_smart_storage_items():** Get a list of all stored items.
- **get_available_smart_storage_items():** Get a list of available items.
- **search_smart_storage_items(query: String):** Search for items based on a query string.
- **sort_items_by_name():** Get items sorted by name.
- **get_item_history(id: u64):** Get the history of changes for a specific item.
- **is_item_available(id: u64):** Check if an item is available.
- **get_item_statistics():** Get statistics about stored items.
- **batch_query(queries: Vec<Query>):** Batch query multiple items.
- **get_paginated_smart_storage_items(limit: usize, offset: usize):** Get paginated items.
- **get_item_transaction_history(id: u64):** Get the transaction history for a specific item.

### Update Functions

- **add_smart_storage_item(item: SmartStorageItemPayload):** Add a new item to the storage.
- **update_smart_storage_item(id: u64, payload: SmartStorageItemPayload):** Update information about an existing item.
- **mark_item_as_available(id: u64):** Mark an item as available.
- **mark_item_as_unavailable(id: u64):** Mark an item as unavailable.
- **delete_smart_storage_item(id: u64):** Delete an item from the storage.
- **bulk_update_smart_storage_items(updates: Vec<(u64, SmartStorageItemPayload)>):** Bulk update multiple items.

## Testing

To run tests, use the following command:

```bash
cargo test
```

## Deployment

To deploy the canister locally, follow these steps:

1. Start the canister:

   ```bash
   dfx start
   ```

2. Deploy the canister:

   ```bash
   dfx deploy
   ```

3. Use the generated canister identifier to interact with the deployed canister.

For additional deployment options and configurations, refer to the [Internet Computer SDK documentation](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html).

## Contributing

Feel free to contribute to the project by submitting issues or pull requests. Follow the standard GitHub flow for contributing.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```
