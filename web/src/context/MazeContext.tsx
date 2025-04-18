import { createContext } from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";

export type MazeStore = {
    userId: string | null;
    maze: string //temp value until we know the data structure
}

const defaultStore: MazeStore = {
    userId: null,
    maze: "this will be a maze"
}

export type MazeContextType = {
    mazeStore: MazeStore, setMazeStore: SetStoreFunction<MazeStore>
}

export const MazeContext = createContext<MazeContextType>()

export const MazeContextProvider = (props: any) => {
    const [mazeStore, setMazeStore] = createStore(defaultStore)

    return (
        <MazeContext.Provider value={{mazeStore, setMazeStore}}>
            {props.children}
        </MazeContext.Provider>
    )
}