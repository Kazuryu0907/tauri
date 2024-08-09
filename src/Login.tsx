import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function Logo(){
  return (
    <a href="#" className="flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white">
      Connect To 
      <img className="w-12 h-12 ml-2" src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d3/OBS_Studio_Logo.svg/1024px-OBS_Studio_Logo.svg.png" alt="logo"/>
    </a>
  );

}

const ButtonComponent = () => {
  const [connectState,setConnectState] = useState(false);
  const [errMessage,setErrMessage] = useState("");
  const navigate = useNavigate();

  const onSubmit = async(e:React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    const formData = new FormData(document.querySelector("form") as HTMLFormElement);
    const host = formData.get("hostname");
    const port = parseInt(formData.get("port") as string);
    const password = formData.get("password");
    console.log(host, port, password);
    setConnectState(true);
    invoke("connect_to_obs", { host, port, password })
    .then(async() => {
      invoke("obs_login_init").then(console.log)
      .catch(e => {setConnectState(false);setErrMessage(e);});
      navigate("/obs");
    })
    .catch((e) => {setConnectState(false);setErrMessage(e)});
  }

  const normalButton = () => {
    return (
      <button type="submit" onClick={onSubmit} 
      className="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
      >Submit</button>
    )
  }
  const disableButton = () => {
    return (
      <button disabled className="flex w-full justify-center rounded-md cursor-not-allowed bg-gray-300 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm ">Connecting</button>
    )
  }


  return (
    <div>
      {connectState ? disableButton() : normalButton()}
      <div className="mt-2 text-red-600 font-bold">{errMessage}</div>
    </div>
  )
}

export function Login() {

  return (
    <div className="container">
      <div className="flex flex-col items-center mt-16 ">
        <Logo/>
      </div>
        <div className="mt-7 sm:mx-auto sm:w-full sm:max-w-sm">
          <form className="space-y-6">
            <div>
              <label htmlFor="hostname" className="block text-sm font-medium leading-6 text-gray-900">
                Hostname
              </label>
              <div className="">
                <input
                  id="hostname"
                  name="hostname"
                  type="text"
                  required
                  defaultValue="localhost"
                  className="block w-full rounded-md border-0 p-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>

            <div>
              <div className="flex items-center justify-between">
                <label htmlFor="port" className="block text-sm font-medium leading-6 text-gray-900">
                  Port
                </label>
              </div>
              <div className="">
                <input
                  id="port"
                  name="port"
                  type="number"
                  defaultValue={4455}
                  required
                  className="block w-full rounded-md border-0 p-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
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
                  className="block w-full rounded-md border-0 p-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>
            <div>
              <ButtonComponent/>
            </div>
          </form>
          </div>
    </div>
  );
}

// export default Login;