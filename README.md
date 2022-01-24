# CrossChainTokenSwapNear

## Взаимодействие с контрактом 

### Перевод из NEAR

Для перевода токенов из NEAR необходимо вызвать метод `ft_transfer_call` у контракта токена, который мы хотим обменять. 
Пример JSON и отправления транзакции из cli в файлах [ft_transfer_call.sh](https://github.com/Cryptorubic/CrossChainTokenSwapNear/blob/master/ft_transfer_call.sh) 
и [ft_transfer_call_nusdt.sh](https://github.com/Cryptorubic/CrossChainTokenSwapNear/blob/master/ft_transfer_call_nusdt.sh) (второй файл для свапа если пользователь выбрал 
транзитный токен).

Параметры:

`receiver_id` - адрес нашего кросс-чейн контракта

`amount` - количество токенов для обмена

`msg` - JSON строка, в которой описано [перечисление](https://github.com/Cryptorubic/CrossChainTokenSwapNear/blob/dab1412af8aee2e5efecf1412543642e136727f9/contract/src/token_receiver.rs#L37) из кода контракта.

Ключи для JSON - названия полей в перечислении и структурах.

### Перевод в NEAR

Для перевода в NEAR необходимо вызвать метод `swap_tokens_to_user_with_fee` у нашего кросс-чейн контракта.
Пример JSON и отправления транзакции из cli в файлах [swap_tokens_to_user_with_fee.sh](https://github.com/Cryptorubic/CrossChainTokenSwapNear/blob/master/swap_tokens_to_user_with_fee.sh) 
и [swap_tokens_to_user_with_fee_banana.sh](https://github.com/Cryptorubic/CrossChainTokenSwapNear/blob/master/swap_tokens_to_user_with_fee_banana.sh) (второй файл для свапа если пользователь хочет получить транзитный токен).

Параметры: 

`params` - структура [SwapFromParams](https://github.com/Cryptorubic/CrossChainTokenSwapNear/blob/dab1412af8aee2e5efecf1412543642e136727f9/contract/src/lib.rs#L64).

`msg` - необходим для случая, когда пользователю требуется не транзитный токен. Сюда необходимо засунуть JSPN строку, в которой описано 
[перечисление](https://github.com/Cryptorubic/CrossChainTokenSwapNear/blob/dab1412af8aee2e5efecf1412543642e136727f9/contract/src/ref_finance_swap_action.rs#L12).
