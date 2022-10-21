<script context="module" lang="ts">
  export const ROUTES = {
    home: '/',
  }
</script>

<script lang="ts">
  import { Route } from 'svelte-navigator'
  import HomePage from './pages/HomePage.svelte'
</script>

<Route path={ROUTES.home}>
  <HomePage />
</Route>
