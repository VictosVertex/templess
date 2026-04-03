<script lang="ts">
	import Inventory from './components/Inventory.svelte';
	import TemplateHeader from './components/TemplateHeader.svelte';

	let { data } = $props();
</script>

<div class="mx-auto flex w-full max-w-7xl flex-col items-center gap-2 px-4 pb-12">
	<TemplateHeader template={data.template} classes={data.classes} />

	<div class="mt-8 grid w-full grid-cols-1 items-start gap-8 lg:grid-cols-[1fr_auto_1fr]">
		<div class="flex w-full flex-col gap-6">
			<div class="rounded-lg border border-white/5 bg-black/20 p-4 backdrop-blur-md">bla</div>
		</div>

		<div class="relative flex w-full justify-center">
			<div
				class="pointer-events-none absolute top-1/2 left-1/2 h-150 w-150 -translate-x-1/2
            -translate-y-1/2 rounded-full bg-primary/5 blur-[100px]"
			></div>

			<Inventory />
		</div>

		<div class="flex w-full flex-col gap-6">
			<div class="rounded-lg border border-white/5 bg-black/20 p-4 backdrop-blur-md">bli</div>
		</div>
	</div>
</div>
