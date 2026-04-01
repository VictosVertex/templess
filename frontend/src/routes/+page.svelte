<script lang="ts">
	import TemplateCreation from './components/template_creation.svelte';
	import TemplateHistory from './components/template_history.svelte';
	let { data } = $props();
</script>

<div class="mx-auto flex max-w-7xl flex-row gap-8">
	<TemplateHistory classes={data.classes} templates={data.templates} />
	<TemplateCreation realms={data.realms} classes={data.classes} />
</div>
