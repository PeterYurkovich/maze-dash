import { useContext } from "solid-js"
import { MazeContext } from "../context/MazeContext"

const TestRoute = () => {
    const { mazeStore } = useContext<any>(MazeContext)

    return (
        <div>{mazeStore.userId ? `Logged in as: ${mazeStore.userId}` : "Not logged in"}</div>
    )
  }
  
export default TestRoute