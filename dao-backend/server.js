require('dotenv').config();
const express = require('express');
const cors = require('cors');
const app = express();
const PORT = process.env.PORT;
const routes = require('./src/routes');

app.use(express.json());

app.use(cors());

app.use('/api', routes);

app.listen(PORT, () => console.log(`Server running on http://localhost:${PORT}`));
