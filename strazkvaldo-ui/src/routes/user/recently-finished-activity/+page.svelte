<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
    import {UI_USER_RECENTLY_FINISHED_ACTIVITY} from '$lib/uiRoutes';

	let { data }: PageProps = $props();
    let recently_finished_activities = data.recently_finished_activities;
</script>

<h1>Dokončené aktivity na uzavretie</h1>

<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>dátum</td>
            <td>uzavri</td>
        </tr>
    </thead>
    <tbody>
{#each recently_finished_activities as finished_activity}
		<tr>
            <td>{finished_activity.code}</td>
            <td>{finished_activity.due_date}</td>
            <td><a href={UI_USER_RECENTLY_FINISHED_ACTIVITY.VIEW(finished_activity.code)}>uzavri</a> </td>
        </tr>
{/each}
    </tbody>
</table>