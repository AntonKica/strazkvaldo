<script lang="ts">
	import type { PageProps } from './$types';
    import { goto } from '$app/navigation';
    import { SVC_USER_ROOM } from '$lib/serviceRoutes';
    import { UI_USER_ROOM } from '$lib/uiRoutes';

	let { data }: PageProps = $props();
	let room_types = data.room_types;
	
    const handleSubmit = async (event: SubmitEvent) => {
        event.preventDefault();

        const formData = new FormData(event.target as HTMLFormElement);
        const formEntries = Object.fromEntries(formData);
		
		fetch(SVC_USER_ROOM.POST(), {
			method: "POST",
            headers: {
                'Content-Type': 'application/json' 
            },
			body:JSON.stringify(formEntries)
		}).then((response) => {
            if(response.ok) {
                goto(UI_USER_ROOM.VIEW(response.headers.get('location')??""), { invalidateAll: true})
            } else {
                alert("Chyba pri vytváraní miestnosti")
            }
        })
	}
</script>

<h1>Vytvor miestnosť</h1>

<br>
<form method="POST" onsubmit={handleSubmit}>
	<label> meno <input name="name" type="text"> </label> <br>
    <label> typ miestnosti 
        <select name="room_type">
			{#each room_types as room_type}
				<option value={room_type.code}>{room_type.text}</option>
			{/each}
        </select>
	 </label> <br>
	<label> popis <textarea name="description"></textarea></label> <br><br>
	<button type="submit">Ulož zmeny</button>
</form>