const express = require("express");
const grpc = require("@grpc/grpc-js");
const grpcProtoLoader = require("@grpc/proto-loader");
const path = require("path");
const cors = require("cors");

const PORT = 50006;
let REMOTE_ENDPOINT = "keyrock.oscur.io:50005";
if (process.env.NODE_ENV === "dev") {
    console.log("Warning: Querying local orderbook");
    REMOTE_ENDPOINT = "127.0.0.1:50005";
}
const REMOTE_REQUEST_URI = "/orderbook.OrderbookAggregator/BookSummary";

/** GRPC client **/
let cache = {};
let packageDefinition = grpcProtoLoader.loadSync(
    path.join(__dirname, "assets/prototype/schema.proto"),
    {
        keepCase: true,
        longs: String,
        enums: String,
        defaults: true,
        oneofs: true,
    }
);
let routeGuide = grpc.loadPackageDefinition(packageDefinition).orderbook;
let client = new routeGuide.OrderbookAggregator(REMOTE_ENDPOINT, grpc.credentials.createInsecure());
// console.log(client);
let stream = client.BookSummary();

const readStream = (stream) => {
    stream.on('data', (data) => {
        cache = data;
        return;
    });
};
readStream(stream);


/** Web server **/
const app = express();
// Don't do this at home ↓
app.use(cors({ origin: "*" }));
app.use("/static", express.static(path.join(__dirname, "assets")));

app.get("/", (req, res, next) => {
    res.sendFile(path.join(__dirname, "/assets/html/index.html"));
});

app.get("/get_quotes", (req, res, next) => {
    res.send({ data: cache });
});

app.listen(PORT, () => {
    console.log(`Server listening on http://127.0.0.1:${PORT}`);
});
