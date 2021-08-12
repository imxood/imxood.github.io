import React from 'react'
import TodoList from "./TodoList"
import TodoForm from "./TodoForm"
import TodoFooter from "./TodoFooter"

class TodoApp extends React.Component {

    state = {
        choosevalue: 1,
        data: this.props.data
    }

    OnAddTodoItem = (newItem) => {
        let newData = this.state.data.concat(newItem)
        this.setState({ data: newData })
    }

    ChooseValue = (id) => {
        this.setState({ choosevalue: id })
    }

    render() {

        const { data } = this.state

        return (
            <div class="ui comments">
                <h1>My ToDoList with React</h1>
                <div class="ui divider"></div>
                <TodoForm AddTodoItem={this.OnAddTodoItem} />
                <TodoList data={data} choosevalue={this.state.choosevalue} />
                <TodoFooter SubmitChooseValue={this.ChooseValue} />
            </div>
        )
    }
}

export default TodoApp
