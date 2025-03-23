<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
    console.log(data);
</script>

<h1>Číselníky</h1>
{#each Object.entries(data.enums) as [e, props]}
<h2>{props.name}</h2>

<table>
    <thead>
        <tr>
            <td>Názov</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each props.values as value}
		<tr>
            <td>{value}</td>
            <td><a href="#">vymaž</a></td>
        </tr>
{/each}
    </tbody>
</table>

<hr>
{/each}
