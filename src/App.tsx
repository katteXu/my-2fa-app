// 用于调试

import { useEffect, useState } from "react";
import logo from "./assets/logo.svg";
import header_bg from "./assets/header.svg";
import footer_bg from "./assets/footer.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Timer from "./components/timer";
import Loading from "./components/loading";

function App() {
  let [code, setCode] = useState<any>("");
  let [time, setTime] = useState<any>("");

  useEffect(() => {
    let time = setInterval(() => {
      handleShowCode();
    }, 1000);

    return () => {
      clearInterval(time);
    };
  }, []);

  const handleShowCode = async () => {
    let result: any = await invoke("get_code");
    if (result && result.length === 2) {
      let [code, time] = result;
      setCode(code);
      setTime(time);
    }
  };

  if (!code) {
    return (
      <div className="flex items-center justify-center h-screen">
        <Loading />
      </div>
    );
  }

  return (
    <div className="bg-gray-100 min-h-screen pt-10">
      {/* logo */}
      <div className="h-[30vh] p-10 flex items-center justify-center">
        <img src={logo} className="w-32" />
        <span
          className="text-xl font-extrabold mt-10 scale-y-95"
          style={{ letterSpacing: "4px" }}
        >
          CLOUDFLARE
        </span>
      </div>

      {/* 数字 */}
      <div className="flex justify-center mt-10 items-center gap-3 font-mono">
        {code.split("").map((item: any, index: number) => (
          <div
            key={index}
            className="text-gray-700 text-6xl"
            style={index === 3 ? { marginLeft: "20px" } : {}}
          >
            <span>{item}</span>
          </div>
        ))}
      </div>

      <div className={`flex mt-20 justify-center`}>
        {/* 倒计时 */}
        <div className="">
          <Timer time={time} width={60} />
        </div>
      </div>

      {/* 头部背景 */}
      <img className="fixed top-0 w-screen" src={header_bg} alt="" />

      {/* 底部背景 */}
      <img className="fixed bottom-0 left-0 right-0" src={footer_bg} alt="" />
    </div>
  );
}

export default App;
