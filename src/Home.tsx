import { useEffect, useState } from "react";
import { Store } from "@tauri-apps/plugin-store";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function Home() {
  let [title, setTitle] = useState<any>("");
  let [secretCode, setSecretCode] = useState<any>("");
  let [code, setCode] = useState<any>("");
  let [time, setTime] = useState<any>("");
  useEffect(() => {
    get_store();

    let time = setInterval(() => {
      handleShowCode();
    }, 1000);

    return () => {
      clearInterval(time);
    };
  }, []);

  const get_store = async () => {
    const store = await Store.load("app_data.json");
    store.entries().then((data) => {
      console.log("data ", data);
    });
  };

  const get_data = async () => {
    const store = await Store.load("app_data.json");
    let data: any = await store.get(title);
    console.log(title, data);
  };

  const add_data = async () => {
    if (!secretCode) {
      alert("Please enter secret code");
      return;
    }
    let result = await invoke("add_data", {
      title: "MyKey",
      secretCode: secretCode,
    });

    console.log("result", result);
  };

  const handleShowCode = async () => {
    let result: any = await invoke("get_code");
    if (result && result.length === 2) {
      let [code, time] = result;
      setCode(code);
      setTime(time);
    }
  };

  return (
    <div className="bg-gray-100 min-h-screen pt-10">
      <h1 className="text-3xl text-[#3d86ef] text-center">
        欢迎访问我的2FA认证
      </h1>
      <div className="flex justify-center mt-10 items-center">
        <label htmlFor="title" className="text-xl mr-2">
          标题
        </label>
        <input
          id="title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          type="text"
          placeholder="请输入标题进行搜索"
          style={{ width: "200px", padding: "5px 10px", fontSize: "16px" }}
          className="border border-gray-300 rounded-md outline-none focus:ring-2 focus:ring-[#3d86ef] focus:border-transparent"
        />
        <button
          className="bg-[#3d86ef] text-white px-3.5 py-1.5 rounded-md hover:bg-[#2c6ae0] transition-colors duration-300 ease-in-out ml-4"
          onClick={get_data}
        >
          搜索
        </button>
      </div>
      <div className="flex justify-center mt-10 items-center">
        <div className="flex justify-center items-center">
          <input
            id="secret_code"
            type="text"
            value={secretCode}
            placeholder="请输入Authentication code"
            onChange={(e) => setSecretCode(e.target.value)}
            style={{ width: "260px", padding: "5px 10px", fontSize: "16px" }}
            className="border border-gray-300 rounded-md outline-none focus:ring-2 focus:ring-[#3d86ef] focus:border-transparent"
          />
        </div>
        <button
          className="bg-[#3d86ef] text-white px-3.5 py-1.5 rounded-md hover:bg-[#2c6ae0] transition-colors duration-300 ease-in-out ml-4"
          onClick={add_data}
        >
          生成
        </button>
      </div>
      <div>
        {/* 6个蓝色的每30s动态改变的数字 */}
        <div className="flex justify-center mt-10 items-center gap-4">
          {code.split("").map((item: any, index: number) => (
            <div
              key={index}
              className="bg-[#3d86ef] text-white p-3.5 rounded-md"
            >
              <span className="text-2xl font-bold">{item}</span>
            </div>
          ))}
        </div>

        {/* 倒计时 时钟形式 */}
        <div className="flex justify-center mt-10 items-center text-xl">
          <div className="text-red-500">{time.toString().padStart(2, "0")}</div>
          <div className="ml-2">秒</div>
        </div>
      </div>
    </div>
  );
}

export default Home;
