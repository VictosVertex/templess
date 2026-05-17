<script lang="ts">
	import TemplateCreation from './components/TemplateCreation.svelte';
	import TemplateHistory from './components/TemplateHistory.svelte';
	let { data } = $props();
</script>

<div
	class="mx-auto flex w-full max-w-400 flex-col items-stretch justify-start gap-12 px-4 pt-12 pb-16 sm:px-8"
>
	<div class="w-full">
		<TemplateCreation realms={data.realms} classes={data.classes} />
	</div>

	<div class="w-full">
		<TemplateHistory classes={data.classes} templates={data.templates} />
	</div>
</div>
