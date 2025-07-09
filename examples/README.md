#  Examples

This directory contains examples demonstrating how to use the Services in the Prime RS SDK.

## Prerequisites

1. **Environment Setup**: Create a `.env` file in the project root with your credentials:
   ```env
   PRIME_CREDENTIALS='{"AccessKey": "", "SecretKey": "", "Passphrase": ""}'
   PORTFOLIO_ID=your_portfolio_id_here
   ENTITY_ID=your_entity_id_here      # Optional, for get_entity examples
   ```

2. **Dependencies**: Make sure you have the required dependencies installed.

## Running Examples

1. **Set up your environment variables** in the `.env` file
2. **Choose an example** based on what you want to demonstrate
3. **Run the example** using `cargo run --example <example_name>`
4. **Review the output** to understand the API response structure

## Troubleshooting

### Common Issues

1. **Authentication Errors**: Check your API credentials in the `.env` file
2. **Missing Environment Variables**: Ensure all required variables are set
3. **Deserialization Errors**: The improved error handling will show the raw response body to help debug type mismatches
4. **Network Errors**: Check your internet connection and API endpoint accessibility

### Debug Mode

All examples include debug logging. To see detailed HTTP requests and responses, set the log level:
```bash
RUST_LOG=debug cargo run --example get_portfolio_activities
```
