<script lang="ts">
	import type { PageProps } from './$types';
    import { base } from '$app/paths';

	let { data }: PageProps = $props();
</script>

<h1>Miestnosti</h1>

<table>
    <thead>
        <tr>
            <td>názov</td>
            <td>typ</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.rooms as room}
		<tr>
            <td>{room.name}</td>
            <td>{room.room_type.text}</td>
            <td><a href="{base}/user/room/{room.code}/view">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>