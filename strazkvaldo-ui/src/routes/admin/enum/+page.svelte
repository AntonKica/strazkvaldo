<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
    console.log(data);
</script>

<h1>Číselníky</h1>

<h2>rola</h2>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.app_user_role as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
            <td><a href="#"></a></td>
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
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.criticality_type as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
            <td><a href="#"></a></td>
        </tr>
{/each}
    </tbody>
</table>
<hr>

<h2>typ miesnosti</h2>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.room_type as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
            <td><a href="#"></a></td>
        </tr>
{/each}
    </tbody>
</table>
<hr>
<h2>typ aktivity</h2>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.activity_type as item}
		<tr>
            <td>{item.code}</td>
            <td>{item.text}</td>
            <td><a href="#"></a></td>
        </tr>
{/each}
    </tbody>
</table>
<hr>
