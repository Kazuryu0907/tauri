import { MemoryRouter as Router, Routes, Route, } from "react-router-dom"
import { Login } from "./Login"
import { OBS } from "./OBS"

export const App = () => {
  return(
    <div>
      <Router>
        <Routes>
          <Route path="/" element={<Login/>} />
          <Route path="/obs" element={<OBS/>}/>
        </Routes>
      </Router>
    </div>
  )
}

