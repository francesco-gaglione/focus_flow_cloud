import { addMessages, init, getLocaleFromNavigator } from 'svelte-i18n'
import en_common from '../../public/locales/en/common.json'
import en_login from '../../public/locales/en/login.json'
import en_todo from '../../public/locales/en/todo.json'
import it_common from '../../public/locales/it/common.json'
import it_login from '../../public/locales/it/login.json'
import it_todo from '../../public/locales/it/todo.json'

let initialized = false

export function setupI18n() {
    if (initialized) return
    initialized = true

    addMessages('en', { common: en_common, login: en_login, todo: en_todo })
    addMessages('it', { common: it_common, login: it_login, todo: it_todo })

    init({
        fallbackLocale: 'en',
        initialLocale: getLocaleFromNavigator() ?? 'en',
    })
}
