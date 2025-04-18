import { lazy, type Component } from 'solid-js';
import styles from './App.module.css';
import { Route, Router } from '@solidjs/router';
import { Header } from './Header';

const Home = lazy(() => import("./routes/Home"));
const TestRoute = lazy(() => import("./routes/TestRoute"))

const App: Component = () => {
  return (
    <div class={styles.Root}>
      <Header />
      <div class={styles.ContentRoot}>
        <Router>
          <Route path="/" component={Home} />
          <Route path="/balls" component={TestRoute} />
        </Router>
      </div>
    </div>
  );
};

export default App;
