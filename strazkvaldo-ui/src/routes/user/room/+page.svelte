<script lang="ts">
	import type { PageProps } from './$types';
    import { UI_USER_ROOM } from '$lib/uiRoutes';

	let { data }: PageProps = $props();
</script>

<h1>Miestnosti</h1>

<a href={UI_USER_ROOM.CREATE()}>Vytvor novú miestnosť &#x2795</a>
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
            <td><a href={UI_USER_ROOM.VIEW(room.code)}>pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>