// server.js
require('dotenv').config();
const express = require('express');
const app = express();
const port = process.env.PORT;

app.use(express.json()); // Middleware to parse JSON

app.get('/', (req, res) => {
  res.send('NEAR DAO API is running...');
});

app.listen(port, () => {
  console.log(`Server running on http://localhost:${port}`);
});
