import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
// import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function connect_to_obs() {
    const host = (document.getElementById("hostname") as HTMLInputElement).value;
    const port = (document.getElementById("port") as HTMLInputElement).value;
    const password = (document.getElementById("password") as HTMLInputElement).value;
    console.log(host, port, password);
    // setGreetMsg(await invoke("connect_to_obs", { hostname, port, password }));
  }
  async function onSubmit(e:React.MouseEvent<HTMLButtonElement>) {
    e.preventDefault();
    const formData = new FormData(document.querySelector("form") as HTMLFormElement);
    const host = formData.get("hostname");
    const port = parseInt(formData.get("port") as string);
    const password = formData.get("password");
    console.log(host, port, password);
    setGreetMsg(await invoke("connect_to_obs", { host, port, password }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <h1>{greetMsg}</h1>
        <div className="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
          <form className="space-y-6">
            <div>
              <label htmlFor="hostname" className="block text-sm font-medium leading-6 text-gray-900">
                Hostname
              </label>
              <div className="mt-2">
                <input
                  id="hostname"
                  name="hostname"
                  type="text"
                  required
                  defaultValue="localhost"
                  className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>

            <div>
              <div className="flex items-center justify-between">
                <label htmlFor="port" className="block text-sm font-medium leading-6 text-gray-900">
                  Port
                </label>
              </div>
              <div className="mt-2">
                <input
                  id="port"
                  name="port"
                  type="number"
                  defaultValue={4455}
                  required
                  className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>

            <div>
              <div className="flex items-center justify-between">
                <label htmlFor="password" className="block text-sm font-medium leading-6 text-gray-900">
                  Password
                </label>
              </div>
              <div className="mt-2">
                <input
                  id="password"
                  name="password"
                  type="password"
                  defaultValue={""}
                  autoComplete="current-password"
                  className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>
            <div>
              <button
                type="submit"
                onClick={onSubmit}
                className="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
              >
                Sign in
              </button>
            </div>
          </form>
          </div>

      {/* <div className="flex justify-center">
        <div className="flex max-w-md min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8">
          <div className="mt-10">
            <form className="space-y-6" action="#" method="POST">
              <div>
                <label className="block text-sm font-medium leading-6 text-gray-900" htmlFor="hostname">Hostname</label>
                <input className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1" type="text" id="hostname" name="hostname" />
              </div>
              <div>
                <label className="block text-sm font-medium leading-6 text-gray-900" htmlFor="port">Port</label>
                <input className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1" type="number" id="port" name="port" />
              </div>
              <div>
                <label className="block text-sm font-medium leading-6 text-gray-900" htmlFor="password">Password</label>
                <input className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1" type="password" id="password" name="password" />
              </div>
            </form>
          </div>
          <button type="submit"
          className="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Connect to OBS</button>
        </div>
      </div> */}
      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
