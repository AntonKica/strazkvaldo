<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Miestnosti</h1>

<table>
    <thead>
        <tr>
            <td>Názov</td>
            <td>Typ</td>
            <td>Popis</td>
        </tr>
    </thead>
    <tbody>
{#each data.rooms as room}
		<tr>
            <td>{room.name}</td>
            <td>{room.type}</td>
            <td>{room.description}</td>
        </tr>
{/each}
    </tbody>
</table>