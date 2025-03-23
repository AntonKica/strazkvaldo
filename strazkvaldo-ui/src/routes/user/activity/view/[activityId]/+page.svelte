<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Aktivita</h1>
<b>Názov</b> {data.name} <br>
<b>Typ</b> {data.activityType} <br>
<b>Popis</b> {data.description}