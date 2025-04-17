import { BiRegularHomeAlt2 } from 'solid-icons/bi';
import styles from './App.module.css';
import { AiOutlineUser } from 'solid-icons/ai';

export const Header = () => {
    return <header class={styles.Header}>
      <div class={styles.HeaderTitle}>
        <a href="/" class={styles.NavIconUnselected}><BiRegularHomeAlt2 class={styles.HeaderIcon} title="Home"/></a>
      </div>
      <div class={styles.HeaderTitle}>
        <h3 class={styles.Title}>Maze-Dash</h3>
      </div>
      <div class={styles.HeaderTitle}>
        <AiOutlineUser class={styles.HeaderIcon} title="idk this can be user info or something"/>
      </div>
    </header>
  }