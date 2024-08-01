import {useState,useEffect} from "react";
import {listen} from "@tauri-apps/api/event";
import useSound from "use-sound";


export function OBS() {
    const [play,{stop,pause}] = useSound("C:\\Users\\kazum\\Videos\\Replay 2024-08-01 01-50-52.mp4")
    const [filenames,setFilename] = useState<String[]>([]);
    useEffect(() => {
        play();
        let unlisten:any;
        async function f(){
            unlisten = await listen("capture_file",event => {
                setFilename([...filenames,event.payload as string]);
                console.log(event.payload);
            })
        }
        f();

        return () => {if(unlisten)unlisten()};
    },[]);

    return(
        <div>
            {filenames}
        </div>
    )
}