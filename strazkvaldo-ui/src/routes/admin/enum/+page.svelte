<script lang="ts">
    import { goto } from '$app/navigation';
    import { UI_ADMIN_ENUM_ROOM_TYPE, UI_ADMIN_ENUM_ACTIVITY_TYPE } from '$lib/uiRoutes';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Číselníky</h1>

<h2>aplikačná rola</h2>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
        </tr>
    </thead>
    <tbody>
{#each data.app_user_role as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
        </tr>
{/each}
    </tbody>
</table>
<hr>

<h2>kritickosť</h2>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
        </tr>
    </thead>
    <tbody>
{#each data.criticality_type as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
        </tr>
{/each}
    </tbody>
</table>

<hr>
<h2>periodicita</h2>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
        </tr>
    </thead>
    <tbody>
{#each data.periodicity as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
        </tr>
{/each}
    </tbody>
</table>
<hr>

<h2>typ miesnosti </h2>
<small>
<button type="button" style="background-color: cadetblue;" onclick={() => goto(UI_ADMIN_ENUM_ROOM_TYPE.EDIT(), { invalidateAll: true})}>uprav číselník</button>
</small>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
        </tr>
    </thead>
    <tbody>
{#each data.room_type as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
        </tr>
{/each}
    </tbody>
</table> 
<hr>
<h2>typ aktivity</h2>
<button type="button" style="background-color: cadetblue;" onclick={() => goto(UI_ADMIN_ENUM_ACTIVITY_TYPE.EDIT(), { invalidateAll: true})}>uprav číselník</button>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
        </tr>
    </thead>
    <tbody>
{#each data.activity_type as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
        </tr>
{/each}
    </tbody>
</table>
<hr>
