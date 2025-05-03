<script lang="ts">
    import { goto } from '$app/navigation';
    import { SVC_ADMIN_ENUM_GENERIC } from '$lib/serviceRoutes';
    import { UI_ADMIN_ENUM } from '$lib/uiRoutes';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
    let enum_values = $state(data.enum_values);
    let name = data.name;
    let enum_name = data.enum_name;
    let new_enum_code = $state("");
    let new_enum_text = $state("");

    function add_enum_item() {
        let code = new_enum_code.trim();
        let text = new_enum_text.trim();
        if(!code) {
            alert("Kód je prázdny.");
            return;
        }            
        if(!text) {
            alert("Text je prázdny.");
            return;
        }            

        for(let enum_value of enum_values) {
            if(enum_value.code === code) {
                alert("Duplicitný názov kódu " + code);
                return;
            }
        }

        enum_values.push({code:code, text:text});
        new_enum_code = "";
        new_enum_text = "";
    }
    function save_enum() {
		fetch(SVC_ADMIN_ENUM_GENERIC(name), {
			method: "POST",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(enum_values)
		}).then((response) => {
            if(response.ok) {
                alert("Číselník bol uložený")
                goto( UI_ADMIN_ENUM, { invalidateAll: true})
            } else {
                alert("Nastala chyba pri ukládaní čiselníka")
            }
        })
    }
</script>

<h2>úprava číselníka {enum_name}</h2>
<button type="button" style="color: green;" onclick={save_enum}>ulož zmeny</button> |
<button type="button" style="color:darkred" onclick={() => goto( UI_ADMIN_ENUM, { invalidateAll: true})}>zahoď zmeny</button>
<table>
    <thead>
        <tr>
            <td>kód</td>
            <td>znenie</td>
            <td></td>
        </tr>
    </thead>
    <tbody>
{#each enum_values as enum_item}
		<tr>
            <td><input type="text" maxlength="16" bind:value={enum_item.code} placeholder="kód novej hodnoty"/></td>
            <td><input type="text" maxlength="20" bind:value={enum_item.text} placeholder="názov novej hodnoty"/></td>
            <td>
                <button type="button" style="color:darkred" onclick={() => enum_values = enum_values.filter(o => o !== enum_item)}>vymaž</button>
            </td>
        </tr>
{/each}
		<tr>
            <td><input type="text" maxlength="16" bind:value={new_enum_code} placeholder="kód novej hodnoty"/></td>
            <td><input type="text" maxlength="20" bind:value={new_enum_text} placeholder="názov novej hodnoty"/></td>
            <td>
                <button type="button" style="color:green" onclick={add_enum_item}>pridaj</button>
            </td>
        </tr>
    </tbody>
</table>