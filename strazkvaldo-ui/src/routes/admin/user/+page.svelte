<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
    console.log(data);
</script>

<h1>Používatelia</h1>
<table>
    <thead>
        <tr>
            <td>kód použivateĺa</td>
            <td>používateľské meno</td>
            <td>meno</td>
            <td>priezvisko</td>
            <td>rola</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each data.users as user}
		<tr>
            <td>{user.code}</td>
            <td>{user.username}</td>
            <td>{user.firstName}</td>
            <td>{user.lastName}</td>
            <td>{user.role}</td>
            <td><a href="#">pozri</a></td>
        </tr>
{/each}
    </tbody>
</table>