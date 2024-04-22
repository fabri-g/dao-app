const express = require('express');
const app = express();
const PORT = process.env.PORT;
const routes = require('./src/routes');

require('dotenv').config();

app.use(express.json());
app.use('/api', routes);

app.listen(PORT, () => console.log(`Server running on http://localhost:${PORT}`));
