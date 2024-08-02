import {useState,useEffect} from "react";
import {listen} from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export function OBS() {
    // const [filenames,setFilename] = useState<string[]>(["C:/Users/kazum/Videos/Replay 2024-08-02 12-37-59.mp4","C:/Users/kazum/Videos/Replay 2024-08-02 12-38-04.mp4","C:/Users/kazum/Videos/Replay 2024-08-02 12-38-04.mp4","C:/Users/kazum/Videos/Replay 2024-08-02 12-38-04.mp4","C:/Users/kazum/Videos/Replay 2024-08-02 12-38-04.mp4"]);
    const [filenames,setFilename] = useState<string[]>([]);
    useEffect(() => {
        let unlisten:any;
        async function f(){
            unlisten = await listen("capture_file",event => {
                setFilename(_f => {
                    let set = new Set([..._f,event.payload as string]);
                    let arr = Array.from(set.values());
                    return arr;
                });
                console.log(event.payload);
            })
        }
        f();

        return () => {if(unlisten)unlisten()};
    },[]);
    
    console.log(filenames);
    // let file_div = filenames.map((filename) => <li key={filename}>{filename}</li>);

    const onClick = async () => {
        invoke("play_vlc_source",{ filenames: filenames })
        .then(() => {console.log("success");setFilename([])})
        .catch((e) => console.log(e));
    }
    const Button = () => {
        return (
        <button type="submit" onClick={onClick}
        className="justify-center rounded-md bg-indigo-600 px-8 py-2 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        >Play</button>
        )
    }
    return(
        <div className="container m-5 mt-16">
            <div className="flex flex-col">
                <div className="flex justify-center"><a className="font-bold text-lg">Clips:{filenames.length}</a></div>
                <div className="mt-3 max-h-[40vh] flex flex-col justify-center overflow-auto">
                    {filenames.length > 0 ? filenames.map((filename) => <div className="rounded-md p-3 border mx-auto inline-block" key={filename}>
                        <p>{filename}</p></div>)
                        : <div className="mt-10 rounded-md p-3 border mx-auto inline-block">No clips</div>}
                </div>

                <div className="mt-16 flex justify-center"><Button/></div>
            </div>

        </div>
    )
}