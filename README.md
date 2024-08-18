# Booking API

This project is a simple booking API, where you can reserve cars. Follow the steps below to set up and run the project on your local machine.

## Prerequisites

Make sure you have the following installed:

- Rust
- MongoDB
- A `.env` file with the required environment variables

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/booking-api.git
   cd booking-api
   ```
   
2. Set up environment variables:

   ```bash
   # Example environment variables
   MONGODB_DSN=mongodb://localhost:27017/default?directConnection=true
   ```

3. Install dependencies:
   
   ```bash
   cargo build
   ```