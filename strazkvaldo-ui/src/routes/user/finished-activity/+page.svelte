<script lang="ts">
    import { base } from '$app/paths';
	import type { PageProps } from './$types';
    import {UI_USER_ONE_TIME_ACTIVITY, UI_USER_REPEATED_ACTIVITY} from '$lib/uiRoutes';

	let { data }: PageProps = $props();
    let finished_activities = data.finished_activities;
</script>

<h1>Dokončené aktivity</h1>

<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>datum</td>
            <td>reviewed</td>
            <td>aktivita</td>
        </tr>
    </thead>
    <tbody>
{#each finished_activities as finished_activity}
		<tr>
            <td>{finished_activity.code}</td>
            <td>{finished_activity.due_date}</td>
            <td>{finished_activity.reviewed}</td>
            <td>
            {#if finished_activity.one_time_activity_code}
                <a href={UI_USER_ONE_TIME_ACTIVITY.VIEW(finished_activity.one_time_activity_code)}>aktivita</a>
            {:else}
                <a href={UI_USER_REPEATED_ACTIVITY.VIEW(finished_activity.repeated_activity_code)}>aktivita</a>
            {/if}
            </td>
        </tr>
{/each}
    </tbody>
</table>