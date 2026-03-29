<script lang="ts">
	import TemplateCreation from './components/template_creation.svelte';
	let { data } = $props();
</script>

<TemplateCreation 
    realms={data.realms} 
    classes={data.classes} 
/>
