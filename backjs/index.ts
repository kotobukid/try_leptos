import express, {type Request, type Response, type NextFunction} from "express";
import cors from "cors";

const app = express();
const port = 3000;

app.use(cors({
  origin: 'http://localhost:8080', // フロントエンドが実行されているURL
  methods: "GET,POST,PUT,DELETE", // 許可するHTTPメソッド
}));

// ProxyのRewriteによりパス部分が「消費」「変換」されることを懸念して用意しておく
app.get("/api/api/hello", (req: Request, res: Response, next: NextFunction) => {
  res.send("Hello World, from Bun with Express! 1");
});

// 本命
app.get("/api/hello", (req: Request, res: Response, next: NextFunction) => {
  res.send("Hello World, from Bun with Express! 2");
});

// 消費対策
app.get("/hello", (req: Request, res: Response, next: NextFunction) => {
  res.send("Hello World, from Bun with Express! 3");
});

app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
});