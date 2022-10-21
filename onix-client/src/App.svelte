<script lang="ts">
  import { Router } from 'svelte-navigator'
  import AppRouter from './AppRouter.svelte'
</script>

<Router primary={false}>
  <AppRouter />
</Router>
