import { Accessor, createSignal, Setter, useContext } from "solid-js"
import styles from "./home.module.css"
import { MazeContext } from "../context/MazeContext"

const Home = () => {
  const { mazeStore, setMazeStore } = useContext<any>(MazeContext)

  const [l1, setL1] = createSignal(mazeStore.userId ? mazeStore.userId[0] : "A")
  const [l2, setL2] = createSignal(mazeStore.userId ? mazeStore.userId[1] : "A")
  const [l3, setL3] = createSignal(mazeStore.userId ? mazeStore.userId[2] : "A")

  const handleConfirm = () => {
    console.log(`User is logging in with initials [${l1()}${l2()}${l3()}]`)
    setMazeStore("userId", () => `${l1()}${l2()}${l3()}`)
    //TODO: route user to maze page with url params
  }

  const handleLetterClick = (letter: Accessor<string>, setLetter: Setter<string>) => {
    const nextLetterCode = letter().charCodeAt(0) + 1
    setLetter(String.fromCharCode(nextLetterCode === 91 ? 65 : nextLetterCode))
  }

  return (
    <div class={styles.Home}>
      <h5>Enter Name</h5>
      <div class={styles.NameTrio}>
        <div class={styles.LetterIconParent}>
          <div onClick={() => handleLetterClick(l1, setL1)} class={styles.LetterIcon}>{l1()}</div>
        </div>
        <div class={styles.LetterIconParent}>
          <div onClick={() => handleLetterClick(l2, setL2)} class={styles.LetterIcon}>{l2()}</div>
        </div>
        <div class={styles.LetterIconParent}>
          <div onClick={() => handleLetterClick(l3, setL3)} class={styles.LetterIcon}>{l3()}</div>
        </div>
      </div>
      <a href={`/balls?user=${l1()}${l2()}${l3()}`}>
        <button class={styles.ConfirmButton} onClick={handleConfirm}>Confirm</button>
      </a>
    </div>
  )
}

export default Home

