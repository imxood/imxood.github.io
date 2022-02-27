import React from 'react'
import Todo from './Todo'

class TodoList extends React.Component {

    render() {
        console.log(this.props)
        var vaule = this.props.choosevalue
        const a = this.props.data.map(({ id, text, complete }, index) => {
            if (vaule === '1') {
                return <Todo ChangeComplete={this.ChangeComplete} key={index} id={id} text={text} complete={complete} />
            }
            if (vaule === '2' && complete === false) {
                return <Todo key={index} id={id} text={text} complete={complete} />
            }
            if (vaule === '3' && complete === true) {
                return <Todo key={index} id={id} text={text} complete={complete} />
            }
            return ""
        })

        return (
            <div>{a}</div>
        )
    }
}


export default TodoList