import type { Component } from 'solid-js';
import styles from './App.module.css';
import { Home } from './routes/Home';
import { TestRoute } from './routes/TestRoute';
import { Route, Router } from '@solidjs/router';
import { Header } from './Header';
import { Title } from "@solidjs/meta"

const App: Component = () => {
  return (
    <div class={styles.Root}>
      <Header />
      <div class={styles.ContentRoot}>
        <Router>
          <Route path="/" component={<Home />} />
          <Route path="/balls" component={<TestRoute />} />
        </Router>
      </div>
    </div>
  );
};

export default App;
