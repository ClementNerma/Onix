<script lang="ts">
  import { Router } from 'svelte-navigator'
  import AppRouter from './AppRouter.svelte'
  import Template from './templates/Template.svelte'
</script>

<Router primary={false}>
  <Template>
    <AppRouter />
  </Template>
</Router>
