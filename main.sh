root_path=$(dirname $(realpath $0))
root_name=$(basename $root_path)

export http_proxy=http://127.0.0.1:1080
export https_proxy=http://127.0.0.1:1080

worksapce='rustiny'
members=(
    'number'
    'fixed_point'
    'linear_algebra'
    'computer_graphic'
    'xfile'
    'renderer'
)

function color_msg() {
    local color=${1:?'(r)ed or (g)reen (b)lue (y)ellow (p)urple (c)yan'}

    if (( 2 > $# )); then
        return
    fi

    if [[ 'r' == $color ]]; then
        echo -e '\033[31m'${@:2}'\033[0m' # red
    elif [[ 'g' == $color ]]; then
        echo -e '\033[32m'${@:2}'\033[0m' # green
    elif [[ 'b' == $color ]]; then
        echo -e '\033[34m'${@:2}'\033[0m' # blue
    elif [[ 'y' == $color ]]; then
        echo -e '\033[33m'${@:2}'\033[0m' # yellow
    elif [[ 'p' == $color ]]; then
        echo -e '\033[35m'${@:2}'\033[0m' # purple
    elif [[ 'c' == $color ]]; then
        echo -e '\033[36m'${@:2}'\033[0m' # cyan
    else
        echo -e '\033[37m'${@:2}'\033[0m' # white
    fi
}

function get_id_name() {
    local source_file=${1:?''}

    local source_file_name=`basename $source_file`

    local id_name=${source_file_name%.*}

    if [[ 'main' == $id_name || 'lib' == $id_name || 'bin' == $id_name ]]; then
        local dir=`dirname $source_file`
        while true; do
            id_name=`basename $dir`
            if [[ 'src' != $id_name ]]; then
                break
            fi
            dir=`dirname $dir`
        done
    fi

    echo $id_name
}

function exec_cargo() {
    local need_input=${1:-'y'}
    local cargo_args=(${@:2})

    if [[ 'y' == $need_input ]]; then
        color_msg y "preparing execute: cargo ${cargo_args[*]}"

        read -p "input cargo ext args: " ext_cargo_args

        for ext_arg in ${ext_cargo_args[@]}; do
            cargo_args+=($ext_arg)
        done
    else
        cargo_args=(${@:1})
    fi

    color_msg g "executing: cargo ${cargo_args[*]}"

    cargo ${cargo_args[@]}

    return $?
}

function select_member() {
    local workspace=${1:-''}

    if [[ -n $workspace ]]; then
        PS3="select(1-$((${#members[@]}+1))): "
        select label in $workspace ${members[@]}; do
            label=${label:-$workspace}
            echo $label
            break
        done
    else
        PS3="select(1-${#members[@]}): "
        select label in ${members[@]}; do
            local index=$(($REPLY-1))
            if (( 0 <= $index && $index < ${#members[@]} )); then
                echo ${members[$index]}
            else
                echo "Illegal Selection: $REPLY" 1>&2
                exit 1
            fi
            break
        done
    fi
}

function select_bin() {
    local root_path_to_build=${1:?''}

    # local main_path=$root_path_to_build/src
    local ext_paths=(
        $root_path_to_build/src/bin
    )

    local builds=()

    # for build in `find $main_path -maxdepth 1 -type f | grep -P 'main.rs$'`; do
    #     builds+=($build)
    # done

    for ext_path in ${ext_paths[@]}; do
        if [[ -d $ext_path ]]; then
            for build in `find $ext_path -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
                builds+=($build)
            done
            for build in `find $ext_path -type f | grep -P 'main.rs$'`; do
                builds+=($build)
            done
        fi
    done

    PS3="select(1-${#builds[@]}): "
    select label in ${builds[@]}; do
        local index=$(($REPLY-1))
        if (( 0 <= $index && $index < ${#builds[@]} )); then
            echo ${builds[$index]}
        else
            echo "Illegal Selection: $REPLY" 1>&2
            exit 1
        fi
        break
    done
}

function select_example() {
    local root_path_to_build=${1:?''}

    # local main_path=$root_path_to_build/src
    local ext_paths=(
        $root_path_to_build/examples
    )

    local builds=()

    # for build in `find $main_path -maxdepth 1 -type f | grep -P 'main.rs$'`; do
    #     builds+=($build)
    # done

    for ext_path in ${ext_paths[@]}; do
        if [[ -d $ext_path ]]; then
            for build in `find $ext_path -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
                builds+=($build)
            done
            for build in `find $ext_path -type f | grep -P 'main.rs$'`; do
                builds+=($build)
            done
        fi
    done

    PS3="select(1-${#builds[@]}): "
    select label in ${builds[@]}; do
        local index=$(($REPLY-1))
        if (( 0 <= $index && $index < ${#builds[@]} )); then
            echo ${builds[$index]}
        else
            echo "Illegal Selection: $REPLY" 1>&2
            exit 1
        fi
        break
    done
}

function cargo_build() {
    echo 'cargo_build'
}

function cargo_run() {
    local member=`select_member`

    local act=
    PS3="select(1-3): "
    select act in 'main' 'bin' 'example'; do
        act=${act:-'main'}
        break
    done

    if [[ 'main' = $act ]]; then
        exec_cargo run -p ${worksapce}_$member
    elif [[ 'bin' == $act ]]; then
        local bin=`select_bin ./$member`
        if [[ -n $bin ]]; then
            exec_cargo run -p ${worksapce}_$member --bin `get_id_name $bin`
        else
            color_msg y 'no bins'
        fi
    elif [[ 'example' == $act ]]; then
        local example=`select_example ./$member`
        if [[ -n $example ]]; then
            exec_cargo run -p ${worksapce}_$member --example `get_id_name $example`
        else
            color_msg y 'no examples'
        fi
    fi
}

function cargo_doc() {
    local member=`select_member workspace`

    if [[ 'workspace' == $member ]]; then
        exec_cargo doc --workspace
    else
        exec_cargo doc -p ${worksapce}_$member
    fi
}

function cargo_publish() {
    exec_cargo y publish -p `select_member`
}

function select_cargo_command() {
    local commands=(
        'build'
        'run'
        'doc'
        'publish'
    )

    PS3="select(1-${#commands[@]}): "
    select label in ${commands[@]}; do
        local index=$(($REPLY-1))
        if (( 0 <= $index && $index < ${#commands[@]} )); then
            cargo_${commands[$index]}
        else
            echo "Illegal Selection: $REPLY" 1>&2
            exit 1
        fi
        break
    done
}

function main() {
    select_cargo_command
}
main